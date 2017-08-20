# POGPEN Playbook

## Usage

To use `pogpen` to create a playbook file, you first need to prepare the {{ parameterVal 'inputs_nomenclature' }}.

### Context File

Parameters are gathered into a context file. See the example
context file for this playbook for usage samples.

Save this file as `{{ parameterVal 'params_file_name' }}`.

{{#if parameters.show_type_info.value}}

The following values as supported for the `type` field:

* Text
* Secret
* Boolean
* Choice
* Number

Samples of each should be provided in the sample.

{{else}}
**NOTE:** *Context file type info hidden due to {{parameterName 'show_type_info'}} being `{{parameterVal 'show_type_info'}}`.*
{{/if}}

### Content File

The content file is a markdown (CommonMark) file that can reference the parameters to
provide the content of the playbook.

Refer to this test data file for usage examples.

Secrets will be shown in plain-text in output,
at this time. (e.g. `{{ parameterVal 'test_secret' }}`)

Save this file as `{{ parameterVal 'content_file_name' }}`.

### Rendering

Use the `pogpen` utility with the input parameters `{{ parameterName 'params_file_name' }}`, `{{ parameterName 'content_file_name' }}`, and `{{ parameterName 'output_file_name' }}`, e.g.:

    pogpen "{{ parameterVal 'params_file_name' }}" "{{ parameterVal 'content_file_name' }}" "{{ parameterVal 'output_file_name' }}"