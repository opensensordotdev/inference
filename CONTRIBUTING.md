# Contributing to Inference

:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:

Feature requests, bug reports, documentation, and source code contributions are welcome.

The following is a set of guidelines for contributing to `inference`. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Table Of Contents

[I don't want to read this whole thing, I just have a question!!!](#i-dont-want-to-read-this-whole-thing-i-just-have-a-question)

[What should I know before I get started?](#what-should-i-know-before-i-get-started)

- [Sensor Organization](#sensor-organization)
- [Important Crates](#important-crates)

[How Can I Contribute?](#how-can-i-contribute)

- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)
- [Pull Requests](#pull-requests)
- [Local Development](#local-development)

[Style](#style)

- [Git Commit Messages](#git-commit-messages)

## I don't want to read this whole thing I just have a question!!!

Please file an issue on the GitHub issues tracker.

## What should I know before I get started?

The core of the `inference` crate is a Rust implementation of the gRPC bindings to [NVIDIA's Triton Inference Server](https://github.com/triton-inference-server/server). `inference` contains the `Model` and `InferenceResult` traits that are used to implement new ML models. Triton allows us to support inference on a variety of models, including TensorRT, TensorFlow, PyTorch, ONNX, OpenVINO, Python, RAPIDS FIL.

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report for `inference`. Following these guidelines helps maintainers and the community understand your report :pencil:, reproduce the behavior :computer: :computer:, and find related reports :mag_right:.

When you are creating a bug report, please [include as many details as possible](#how-do-i-submit-a-good-bug-report). Fill out [the required template](https://github.com/opensensordotdev/inference/blob/main/.github/ISSUE_TEMPLATE/issue.md), the information it asks for helps us resolve issues faster.

> **Note:** If you find a **Closed** issue that seems like it is the same thing that you're experiencing, open a new issue and include a link to the original issue in the body of your new one.

#### How Do I Submit A (Good) Bug Report?

Bugs are tracked as [GitHub issues](https://guides.github.com/features/issues/). Create an issue and provide the following information by filling in [the template](https://github.com/opensensordotdev/inference/blob/main/.github/ISSUE_TEMPLATE/issue.md).

Explain the problem and include additional details to help maintainers reproduce the problem:

- **Use a clear and descriptive title** for the issue to identify the problem.
- **Describe the exact steps which reproduce the problem** in as many details as possible. For example, start by explaining your environment setup, including Rust version you're using. Please include exactly which command you used in the terminal, or how you started `inference` otherwise. When listing steps, **don't just say what you did, but explain how you did it**.
- **Provide specific examples to demonstrate the steps**. Include links to files or GitHub projects, or copy/pasteable snippets, which you use in those examples. If you're providing snippets in the issue, use [Markdown code blocks](https://help.github.com/articles/markdown-basics/#multiple-lines).
- **Describe the behavior you observed after following the steps** and point out what exactly is the problem with that behavior.
- **Explain which behavior you expected to see instead and why.**
- **Include screenshots and animated GIFs** which show you following the described steps and clearly demonstrate the problem. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux.
- **If you're reporting that `inference` crashed**, include a crash report with a stack trace from the operating system. Include the crash report in the issue in a [code block](https://help.github.com/articles/markdown-basics/#multiple-lines), a [file attachment](https://help.github.com/articles/file-attachments-on-issues-and-pull-requests/), or put it in a [gist](https://gist.github.com/) and provide link to that gist.
- **If the problem wasn't triggered by a specific action**, describe what you were doing before the problem happened and share more information using the guidelines below.

Provide more context by answering these questions:

- **Did the problem start happening recently** (e.g. after updating to a new version of ) or was this always a problem?
- If the problem started happening recently, **can you reproduce the problem in an older version of `inference`?** What's the most recent version in which the problem doesn't happen? You can download older versions of `inference` from [the releases page](https://github.com/opensensordotdev/inference/releases).
- **Can you reliably reproduce the issue?** If not, provide details about how often the problem happens and under which conditions it normally happens.
- If the problem is related to working with files (e.g. opening and editing files), **does the problem happen for all files and projects or only some?** Does the problem happen only when working with local or remote files (e.g. on network drives), with files of a specific type (e.g. only JavaScript or Python files), with large files or files with very long lines, or with files in a specific encoding? Is there anything else special about the files you are using?

Include details about your configuration and environment:

- **Which version of `inference` are you using?**
- **What's the name and version of the OS you're using**?
- **Are you running `inference` in a virtual machine or cloud environment?** If so, which VM software/cloud provider are you using and which operating systems and versions are used for the host and the guest?

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for `inference`, including completely new features and minor improvements to existing functionality. Following these guidelines helps maintainers and the community understand your suggestion :pencil: and find related suggestions :mag_right:.

Before creating enhancement suggestions, please check [this list](#before-submitting-an-enhancement-suggestion) as you might find out that you don't need to create one. When you are creating an enhancement suggestion, please [include as many details as possible](#how-do-i-submit-a-good-enhancement-suggestion). Fill in [the template](https://github.com/opensensordotdev/inference/blob/main/.github/ISSUE_TEMPLATE/issue.md), including the steps that you imagine you would take if the feature you're requesting existed.

#### Before Submitting An Enhancement Suggestion

- **Perform a cursory search of existing issues** to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.

#### How Do I Submit A (Good) Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://guides.github.com/features/issues/). Create an issue and provide the following information:

- **Use a clear and descriptive title** for the issue to identify the suggestion.
- **Provide a step-by-step description of the suggested enhancement** in as many details as possible.
- **Provide specific examples to demonstrate the steps**. Include copy/pasteable snippets which you use in those examples, as [Markdown code blocks](https://help.github.com/articles/markdown-basics/#multiple-lines).
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why.
- **Include screenshots and animated GIFs** which help you demonstrate the steps or point out the part of `inference` which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux.
- **Explain why this enhancement would be useful** to most `inference` users.
- **Specify which version of `inference` you're using.**
- **Specify the name and version of the OS you're using.**

### Pull Requests

The process described here has several goals:

- Maintain `inference`'s quality
- Fix problems that are important to users
- Engage the community in working toward the best possible `inference`
- Enable a sustainable system for `inference`'s maintainers to review contributions

Please follow these steps to have your contribution considered by the maintainers:

1. Follow all instructions in [the template](https://github.com/opensensordotdev/inference/blob/main/.github/pull_request_template.md)
2. Follow the [styleguides](./references/STYLE.md)
3. After you submit your pull request, verify that all [status checks](https://help.github.com/articles/about-status-checks/) are passing. *What if the status checks are failing?* If a status check is failing, and you believe that the failure is unrelated to your change, please leave a comment on the pull request explaining why you believe the failure is unrelated. A maintainer will re-run the status check for you. If we conclude that the failure was a false positive, then we will open an issue to track that problem with our status check suite.

While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional design work, tests, or other changes before your pull request can be ultimately accepted.

### Local development

`inference` can be developed locally.

It is possible to develop against a k3s cluster and the zarf package or the `docker compose` infrastructure.

   1. The convenience `bootsrap_cluster.sh` script ensures that the containers come up in the correct order (and adding pauses so containers are truly ready before starting containers that depend on them). It also ensures all the correct topics are set up with the correct number of partitions and segment sizes.
   2. To persist data stored in the volumes, run `docker compose stop`. This makes the containers restartable with `./bootstrap_cluster`.
   3. To discard the data stored in the volumes, run `docker compose down`. This destroys all containers & volumes associated with the compose. To restart after running `docker compose down`, it will be necessary to run `./bootstrap_cluster.sh` again to rebuild the containers & volumes.

## Style

See [style guidelines](./references/STYLE.md) for detailed commit message + Rust formatting guidelines.

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less and include one of the following features (ex. `Docs: added commit message guidance`)
    - Feat: feature
    - Fix: bug fixes
    - Docs: changes to the documentation like README
    - Style: style or formatting change 
    - Perf: improves code performance
    - Test: test a feature
- Reference issues and pull requests liberally after the first line