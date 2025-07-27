# Concourse-CI ntfy.sh Notification Resource

this custom resource allows you to send notifications to your Webbrowser, Smartphone or anything that supports WebPush via [ntfy.sh](https://ntfy.sh) (hosted or self-hosted).

The currently supported parameters are: 
- topic
- title
- message
- priority
- tags

Please reference the example usage on how to use them.

## Example Usage
```yaml
resource_types:
- name: ntfy-resource
  type: docker-image
  source:
    repository: gailingmic/concourse-ntfy-resource
    tag: 0.0.1

resources:
- name: ntfy-resource
  type: ntfy-resource
  source:
    host: https://ntfy.sh
    username: ((ntfy.username))
    password: ((ntfy.password))

jobs:
- name: test-notification
  plan:
  - put: ntfy-resource
    params:
      topic: testtopic
      title: This is a title
      message: This is a message
      priority: 5
      tags:
        - warning
        - skull
```