# POGPEN Playbook

## Usage

To use `pogpen` to create a playbook file, you first need to prepare the inputs.

### Parameters File

Parameters are gathered into a file as such:

    parameters:
      params_file_name:
        name: Parameters File
        type: string
        value: params.yml
      content_file_name:
        name: Content File
        type: string
        value: content.md
      output_file_name:
        name: Output File
        type: string
        value: output.html
      fake_password:
        name: Unused password
        type: password
        value: c0ntent_obscured

Save this file as `{{ parameterVal 'params_file_name' }}`.

The following values as supported for the `type` field:

* string
* password (obscures the input in the parameters pane *only*)

### Content File

The content file is a markdown (CommonMark) file that can reference the parameters to
provide the content of the playbook.

An example:

    # Hello World!

    Here's an example of using an input parameter's value:
    \{{ parameterVal 'params_file_name' }}.

    The value above will be replaced with whatever the value
    of the `\{{ parameterName 'params_file_name' }}` parameter.

Save this file as `{{ parameterVal 'content_file_name' }}`.

### Rendering

Use the `pogpen` utility with the input parameters `{{ parameterName 'params_file_name' }}`, `{{ parameterName 'content_file_name' }}`, and `{{ parameterName 'output_file_name' }}`, e.g.:

    pogpen "{{ parameterVal 'params_file_name' }}" "{{ parameterVal 'content_file_name' }}" "{{ parameterVal 'output_file_name' }}"