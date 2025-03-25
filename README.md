# Defect

![license](https://img.shields.io/github/license/DiscreteTom/defect?style=flat-square)
[![release](https://img.shields.io/github/v/release/DiscreteTom/defect?style=flat-square)](https://github.com/DiscreteTom/defect/releases/latest)

Call LLMs in your pipeline, e.g. local [git hook](#git-hook), [GitHub Actions](#github-actions) and more.

## Features

- Single static-linked binary executable. You don't need to be familiar with Python or any framework.
- Customizable prompt. Suitable for all kinds of tasks. See [examples](#prompt-engineering) below.
- Support OpenAI (or compatible) and AWS Bedrock models.

## Installation

Just download the pre-built binary. E.g. for Linux x86_64 environment:

```bash
wget https://github.com/DiscreteTom/defect/releases/download/v0.3.3/defect-v0.3.3-x86_64-unknown-linux-musl.zip
unzip defect-v0.3.3-x86_64-unknown-linux-musl.zip
rm defect-v0.3.3-x86_64-unknown-linux-musl.zip
chmod +x defect
```

See the [latest GitHub releases](https://github.com/DiscreteTom/defect/releases/latest) page for more pre-built binaries.

> [!IMPORTANT]
> Remember to include the binary in your `PATH`.

## Usage

```bash
$ defect --help
Call LLMs in your pipeline, print the text response to stdout

Usage: defect [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  The prompt to use. If not provided or equal to "-", the program will read from stdin

Options:
  -m, --model <MODEL>    The model to use [default: gpt-4o]
  -s, --schema <SCHEMA>  The API schema to use [default: openai] [possible values: openai, bedrock]
  -S, --system <SYSTEM>  Optional system instructions
  -h, --help             Print help
  -V, --version          Print version
```

### Choose a Model

```bash
# You can use `--model` to specify a custom OpenAI model.
# Make sure you have set the "OPENAI_API_KEY" environment variable.
export OPENAI_API_KEY=""
defect "who are you"
defect --model=gpt-4o "who are you"

# For OpenAI compatible models, e.g. OpenRouter,
# specify a custom endpoint via the "OPENAI_API_BASE" environment variable.
# Make sure you have also set the "OPENAI_API_KEY" environment variable.
export OPENAI_API_BASE="https://openrouter.ai/api/v1"
defect --model=deepseek/deepseek-r1 "who are you"

# For AWS Bedrock models, set the `schema` option.
# Make sure you have AWS credentials set up.
defect --schema bedrock --model=anthropic.claude-3-5-sonnet-20240620-v1:0 "who are you"
```

> To set AWS credentials, you can [use environment variables](https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-envvars.html), or [use AWS CLI](https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-files.html#cli-configure-files-methods). See AWS official documentations for more information.

## Prompt Engineering

The functionality of this tool is highly dependent on the prompt you provide.

You can construct complex prompts in bash scripts and pass it to the tool.

```bash
# insert the content of a file by using string interpolation in bash
prompt="Summarize the file. <file>`cat README.md`</file>"
defect "$prompt"
```

Here are some prompt examples.

<details open>
<summary>General Code Review</summary>

```bash
prompt="
You are a coding expert.
Review the following code and give me suggestions.

<code>
`cat main.rs`
</code>
"
```

</details>

<details>
<summary>Code Review with a Guideline</summary>

Your team may have a coding guideline. You can insert the guideline into the prompt.

```bash
prompt="
You are a coding expert.
Review the code below following my provided guideline
and give me suggestions.

<guideline>
`cat guideline.md 2>/dev/null || curl https://your-server.com/guideline.md`
</guideline>

<code>
`cat main.rs`
</code>
"
```

</details>

<details>
<summary>Document Validation</summary>

```bash
# review comments
prompt="
You are a coding expert.
Review the following code, ensure the comments adheres to the functionality of the code.
If not, provide suggestions to update the comments.

<code>
`cat main.rs`
</code>
"

# review documentation
prompt="
You are a coding expert.
Review the following code, ensure the provided documentation adheres to the functionality of the code.
If not, provide suggestions to update the documentation.

<documentation>
`cat documentation.md`
</documentation>

<code>
`cat main.rs`
</code>
"
```

</details>

## Workflow

### Abort a Workflow Execution

When using this tool in a workflow or pipeline, you may want to abort the execution if the result is not as expected.
This requires your LLM output is structured in a way that you can parse it.

A simple example:

```bash
prompt="
...

If you think the code is correct, output 'OK' with nothing else.
Otherwise, output suggestions in markdown format.
"

output=$(defect "$prompt")

if [ "$output" != "OK" ]; then
  echo "$output"
  exit 1
fi
```

### Webhook Callback

If your workflow execution is aborted by LLM, you may want to send a webhook callback to, for example, Slack, Lark, or your own issue tracker.

```bash
...

if [ "$output" != "OK" ]; then
  echo "$output"

  commit=$(git rev-parse HEAD)
  escaped_output=$(jq -n --arg val "$output" '$val')
  body="{\"commit\":\"$commit\",\"feedback\":$escaped_output}"
  curl -X POST -d "$body" https://your-server.com/webhook

  exit 1
fi
```

### Git Hook

You can review your code locally via git hooks to reduce the feedback loop time, instead of waiting for CI/CD.

An example of `pre-commit` hook:

```bash
#!/bin/sh

if git rev-parse --verify HEAD >/dev/null 2>&1
then
  against=HEAD
else
  # Initial commit: diff against an empty tree object
  against=$(git hash-object -t tree /dev/null)
fi

# Get the diff of the staged files with 100 lines of context
diff=$(git diff --cached -U100 $against)

prompt="
You are a coding expert.
Review the following code diff and give me suggestions.

If you think the code is correct, output 'OK' with nothing else.
Otherwise, output suggestions in markdown format.

<diff>
$diff
</diff>
"

output=$(defect "$prompt")

if [ "$output" != "OK" ]; then
  echo "$output"
  exit 1
fi
```

### GitHub Actions

There is a [setup-defect](https://github.com/DiscreteTom/setup-defect) action available to simplify the setup process.

Review each commit:

```yaml
on:
  push:
    # only trigger on branches, not on tags
    branches: "**"

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      # fetch at least 2 commits so we can get the diff of the latest commit
      - uses: actions/checkout@v4
        with:
          fetch-depth: 2

      # get the diff of the latest commit with 100 lines of context
      - name: Get the diff
        run: |
          git diff -U100 HEAD^ HEAD > /tmp/diff
          cat /tmp/diff

      - name: Setup defect
        uses: DiscreteTom/setup-defect@v0.1.1
        with:
          version: "0.3.3"

      - name: Review the diff using defect
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: |
          diff=$(cat /tmp/diff)

          prompt="
          You are a coding expert.
          Review the following code diff and give me suggestions.

          If you think the code is correct, output 'OK' with nothing else.
          Otherwise, output suggestions in markdown format.

          <diff>
          $diff
          </diff>
          "

          defect "$prompt" > /tmp/suggestions
          cat /tmp/suggestions

      - name: Abort if suggestions are not empty
        run: |
          suggestions=$(cat /tmp/suggestions)

          if [ "$suggestions" != "OK" ]; then
            exit 1
          fi
```

Review each PR (get the diff between the PR branch and the base branch):

```yaml
on:
  pull_request:
    types: [opened, synchronize]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Fetch the origin/main branch
        run: |
          git fetch origin main:refs/remotes/origin/main

      - name: Get the diff
        run: |
          git diff origin/main HEAD > /tmp/diff
          cat /tmp/diff

      # ...
```

## Telemetry

Currently this project doesn't emit any telemetry data.

To collect the LLM usage data, you can use an AI gateway like [OpenRouter](https://openrouter.ai/), [LiteLLM](https://www.litellm.ai/) or [Kong](https://konghq.com/).

To collect the LLM response data, just send the response to your own server or write to your own database.

```bash
...

if [ "$output" != "OK" ]; then
  echo "$output"

  # e.g. send telemetry with a webhook callback
  curl -X POST -d "some-data" https://your-server.com/webhook

  # e.g. convert output to metrics using LLM
  # and save to AWS S3 so you can query using AWS Athena
  metrics_prompt="
  Below is a code review feedback,
  tell me how many suggestions are there.
  You should output a JSON object with the following format:

  <format>
  {\"suggestions\": 123}
  </format>

  You should only output the JSON object with nothing else.

  <feedback>
  $output
  </feedback>
  "
  metrics=$(defect "$metrics_prompt")
  timestamp=$(date +%s)
  echo "$metrics" > $timestamp.json
  date=$(date +'%Y/%m/%d')
  author=$(git log -1 --pretty=format:'%an')
  aws s3 cp $timestamp.json "s3://your-bucket/suggestions/$date/$author/$timestamp.json"

  exit 1
fi
```

## Cost Optimization

Tips to save your money:

- Prevent unnecessary calls. E.g. in code review scenarios, use traditional formatters and linters before calling LLMs.
- Use cheaper models for frequent tasks. E.g. review commits with DeepSeek models while review PRs with Claude models.

## Demo

See [`defect-demo`](https://github.com/DiscreteTom/defect-demo) for a demo project.

## Debug

This project uses [EnvFilter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) to filter logs.
All the logs will be printed to `stderr`.

Here is an example to enable debug logs:

```bash
export RUST_LOG="defect=debug"
```

## [CHANGELOG](./CHANGELOG.md)
