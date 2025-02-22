# Defect

![license](https://img.shields.io/github/license/DiscreteTom/defect?style=flat-square)
[![release](https://img.shields.io/github/v/release/DiscreteTom/defect?style=flat-square)](https://github.com/DiscreteTom/defect/releases/latest)

Call LLM in your pipeline.

## Features

- Single binary executable. You don't need to be familiar with Python.
- Customizable prompt. Suitable for all kinds of tasks. See [examples](#prompt-engineering) below.
- Support OpenAI (or compatible) and AWS Bedrock models.

## Installation

```bash
wget https://github.com/DiscreteTom/defect/releases/latest/download/defect
```

See the [latest GitHub releases](https://github.com/DiscreteTom/defect/releases/latest) page for more pre-built binaries.

## Usage

```bash
$ defect --help
Call LLM in your pipeline. To set an API key, use the "API_KEY" environment variable

Usage: defect [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  The prompt to use. If not provided or equal to "-", the program will read from stdin

Options:
  -m, --model <MODEL>        The model to use. For AWS Bedrock models, use the format "bedrock/<model-id>" [default: gpt-4o]
  -e, --endpoint <ENDPOINT>  The endpoint to use. Only used for OpenAI (or compatible) models [default: https://api.openai.com/v1]
  -h, --help                 Print help
  -V, --version              Print version
```

### Choose a Model

```bash
# for OpenAI models, make sure you have set the "API_KEY" environment variable
defect "who are you"
defect --model=gpt-4o "who are you"

# for OpenAI compatible models, e.g. OpenRouter
defect --model=deepseek/deepseek-r1 --endpoint=https://openrouter.ai/api/v1 "who are you"

# for AWS Bedrock models, make sure you have AWS credentials set up
defect --model=bedrock/anthropic.claude-3-5-sonnet-20240620-v1:0 "who are you"
```

### Prompt Engineering

The functionality of this tool is highly dependent on the prompt you provide.

You can construct complex prompts in bash scripts and pass it to the tool.

```bash
prompt="Summarize the file. <file>`cat README.md`</file>"
defect $prompt
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
<summary>Code Review with Guideline</summary>

```bash
prompt="
You are a coding expert.
Review the following code following my provided guideline
and give me suggestions.

<guideline>
`cat guideline.md`
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

### Workflow

#### Abort a Workflow Execution

When using this tool in a workflow or pipeline, you may want to abort the execution if the result is not as expected.
This requires your LLM output is structured in a way that you can parse it.

A simple example:

```bash
prompt="
...

If you think the code is correct, output 'OK' with nothing else.
Otherwise, output suggestions in markdown format.
"

output=`defect $prompt`

if [ $output != "OK" ]; then
  echo $output
  exit 1
fi
```

#### Webhook Callback

If your workflow execution is aborted by LLM, you may want to send a webhook callback to, for example, Slack, Lark, or your own issue tracker.

```bash
...

if [ $output != "OK" ]; then
  echo $output
  curl -X POST -d "message=$output" https://your-server.com/webhook
  exit 1
fi
```

#### Git Hook

An example of `pre-commit` hook:

```bash
if git rev-parse --verify HEAD >/dev/null 2>&1
then
  against=HEAD
else
  # Initial commit: diff against an empty tree object
  against=$(git hash-object -t tree /dev/null)
fi

# Get the diff of the staged files with 100 lines of context
diff=`git diff --cached -U100 $against`

prompt="
You are a coding expert.
Review the following code diff and give me suggestions.

If you think the code is correct, output 'OK' with nothing else.
Otherwise, output suggestions in markdown format.

<diff>
$diff
</diff>
"

output=`defect $prompt`

if [ $output != "OK" ]; then
  echo $output
  exit 1
fi
```

#### GitHub Actions

```yaml
# download the latest defect binary
- run: |
    wget https://github.com/DiscreteTom/defect/releases/latest/download/defect

# get the diff of the latest commit
- run: |
    git diff -U100 HEAD^ HEAD > diff

# or, get the diff of a PR
- run: |
    git diff -U100 origin/master HEAD > diff

# review the diff
- run: |
    diff=`cat diff`

    prompt="
    You are a coding expert.
    Review the following code diff and give me suggestions.

    If you think the code is correct, output 'OK' with nothing else.
    Otherwise, output suggestions in markdown format.

    <diff>
    $diff
    </diff>
    "

    output=`defect $prompt`

    if [ $output != "OK" ]; then
      echo $output
      exit 1
    fi
  env:
    API_KEY: ${{ secrets.API_KEY }}
```

<!-- TODO: add an AWS Lambda example -->

### Telemetry

Currently this project doesn't emit any telemetry data.

To collect the LLM usage data, you can use an AI gateway like [OpenRouter](https://openrouter.ai/), [LiteLLM](https://www.litellm.ai/) or [Kong](https://konghq.com/).

To collect the LLM response data, just send the response to your own server or write to your own database using something like a [webhook](#webhook-callback).

<!-- TODO: example -->

## Debug

This project uses [EnvFilter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) to filter logs.
All the logs will be printed to `stderr`.

Here is an example to enable debug logs:

```bash
export RUST_LOG="defect=debug"
```

## [CHANGELOG](./CHANGELOG.md)
