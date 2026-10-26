# Host policy publication

Host approvals and per-image manifests are maintained and published in
[nodana-host](https://github.com/nodana/nodana-host), separately from Phoenixd
bundle releases.

See [Host policy releases](https://github.com/nodana/nodana-host/blob/main/docs/host-policy-releases.md)
for environment setup, signing-key configuration, policy publication and
migration from the original Phoenixd-hosted policy.

The CLI 0.1.2 download URL is:

`https://github.com/nodana/nodana-host/releases/download/host-policy-v1/nodana-host-policy.json`

Before tagging 0.1.2, publish the reviewed host policy through that repository
and verify the new CLI against production. Host and bundle checksum flags may
be supplied independently or together; at least one is required.

The host workflow uses `HOST_POLICY_TOOL_COMMIT` in the host repository to pin
the reviewed CLI signing/export tool. Changing the download location does not
change host manifest bytes or checksums, and requires no running-host changes.
