# Publishing host approvals for CLI 0.1.1

Host approvals are signed and published only when the approved images change.
There is no expiry, renewal, scheduled workflow or local cache in the normal CLI
flow. Each `nd attest` downloads the currently published policy over HTTPS:

`https://github.com/nodana/nodana-phoenixd/releases/download/host-policy-v1/nodana-host-policy.json`

The CLI verifies its signature against `policy/signing-public-key.pem` before
using its measurements. Freshness, challenge, AWS signature, TLS binding and
bundle approval checks on the node's attestation remain unchanged.

The signing key stays in the existing `nodana-phoenixd` `bundle-release`
environment. Policy signatures use the separate `nodana-host-policy-v1` domain.
No host, Phoenixd bundle or AWS image rebuild, or website deployment, is needed
to publish policy changes.

## Initial setup and publication

1. Review and commit/push the latest CLI changes and
   `nodana-phoenixd/.github/workflows/sign-host-policy.yml`. Do not tag
   `v0.1.1` yet.
2. In the Phoenixd repository settings, create the **host-policy-approval**
   environment and configure **Required reviewers**. The workflow checks that
   this review gate exists before signing.
3. From the CLI checkout at the reviewed, pushed commit, pin the policy tool
   and start the publication workflow:

   ```sh
   cli_commit=$(git rev-parse HEAD)
   gh variable set HOST_POLICY_TOOL_COMMIT --repo nodana/nodana-phoenixd --body "$cli_commit"
   gh workflow run sign-host-policy.yml --repo nodana/nodana-phoenixd \
     -f cli_source_commit="$cli_commit"
   ```

4. In the GitHub UI, open **Actions → Publish reviewed host approval policy →
   Run workflow** and supply only the exact CLI commit (or use the command above).
   The preparation job calculates the checksum and displays it with a link to
   the exact policy file in its run summary, before the approval job can start.
   Review that file, then approve the pending job. The publishing job rechecks
   the calculated checksum before signing the policy,
   publishes it to the dedicated `host-policy-v1` release, then downloads and
   verifies the public file. It leaves the latest node bundle release unchanged.
   Nothing needs to be copied into the website.
5. Confirm the full download and attestation path using the new CLI:

   ```sh
   env -u NODANA_HOST_POLICY_FILE cargo run --locked -- attest \
     --url https://lark-3xii.nodana.app \
     --approve-release-sha256 a4a0855395db9b0996b8fa8eb7ad95ebbd40a21f675281ac73c5038162b21e29
   ```

6. After that succeeds, tag/push the CLI commit as `v0.1.1`. The existing CLI
   Release workflow builds and publishes the platform binaries. Install the
   released binary and repeat the check with `nd attest`.

## Changing approvals

Use the existing hardware verification and `ndn-infra image-policy` process.
Compare boot measurements with the exact candidate build and Secure Boot
enrollment; never approve measurements solely from a node's live response.

Update `policy/host-images.json`, increment `serial`, and commit/push the
reviewed data. Run the same publication workflow with that commit; its preparation
job calculates and displays the checksum for review before approval.
The trusted signing tool stays at `HOST_POLICY_TOOL_COMMIT`; change that variable
only when reviewing and updating the tool itself.

Remove obsolete image entries when replacing approvals. A signed empty image
list withdraws every approval. The workflow replaces the published policy;
users fetch it on their next attestation without a CLI update. There are no
background jobs or periodic maintenance runs.

Only independently verified production `ami-03f3b551d17c25f49` is included
initially. Staging needs its own verification before inclusion. A local
`NODANA_HOST_POLICY_FILE` still requires a trusted signature. Download failures
stop attestation rather than falling back to an old local policy.

An old saved policy retains a valid signature. Without expiry or a persistent
client rollback counter, its signature alone cannot establish that it is the
latest policy. Normal clients use the currently published policy over HTTPS.
The serial is an audit identifier; signing-key rotation requires a CLI release.

## Production verification on 26 October 2026

AWS identified Lark as `i-05a7915cca36df129`, running
`ami-03f3b551d17c25f49` from the `protected-host-production-v0.1.0` import record.
Its AMI UEFI enrollment matched the release's `uefi_data.aws` byte for byte;
runtime PK, KEK and db matched the enrollment.

PCR4 matched `tpm_pcr.json`. PCR7 was independently reconstructed from
SecureBoot=1, enrolled PK/KEK/db, empty dbx, the separator and the 48-byte db
authority entry, and matched the attested PCR7. PCR12 was zero. Build manifest,
provenance and predicted-PCR file checksums matched `SHA256SUMS`; provenance and
enrollment digests matched their manifest records.

Installed `nd` 0.1.0 then verified a fresh production proof using the independently
checked temporary policy: AWS signature/chain, freshness, nonce, TLS binding,
node identity and the independently obtained bundle checksum all passed. The
original failure was the CLI's stale compiled-in approval. No host or live policy
was changed during investigation. The new signed publication path must pass
step 5 above after initial publication before 0.1.1 is released.
