# POGPEN Playbook

## Usage

To use `pogpen` to create a playbook file, you first need to prepare the inputs.

### Parameters File

Parameters are gathered into a file as such:

    parameters:
      parametersFileName:
        name: Parameters File
        type: string
        secure: false
        value: params.yml
      contentFileName:
        name: Content File
        type: string
        value: content.md
      outputFileName:
        name: Output File
        type: string
        value: output.html

Save this file as `{{ parameterValue parametersFileName }}`.

### Content File

The content file is a markdown (CommonMark) file that can reference the parameters to
provide the content of the playbook.

An example:

    # Hello World!

    Here's an example of using an input parameter's value: \{{ parameterValue parametersFileName }}.

    The value above will be replaced with whatever the value of the `\{{ parameterName parametersFileName }}` parameter.

Save this file as `{{ parameterValue contentFileName }}`.

### Rendering

Use the `pogpen` utility with the input parameters `{{ parameterName parametersFileName }}`, `{{ parameterName contentFileName }}`, and `{{ parameterName outputFileName }}`, e.g.:

    pogpen "{{ parameterValue parametersFileName }}" "{{ parameterValue contentFileName }}" "{{ parameterName outputFileName }}"