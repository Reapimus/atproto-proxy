# ATProto Proxy
This is a WIP service for proxying ATProto blobs and transforming them for the requesting client.

When signing is enabled, all routes expect a signature to be provided via the `sig` query parameter. Signatures should be the path of the route signed using sha256 with the signing key providing in the config.

## Configuration
Various configuration options are provided for this proxy so that it can be set up however it is needed. An example configuration file is provided in `Rocket.example.toml`.

If the `blob_cache` feature flag is enabled, there will be options for configuring the cache provided under the `cache` field in the config file. Similarly, if the `signing` feature flag is enabled, you will be able to provide `core.signingKey` a hex-encoded secp256k1 private key to enforce all endpoints provide a signature.

## Image Proxy
`/img/<did>/<cid>@<parameters?>`

ex. `https://example.com/img/did:plc:ui5pgpumwvufhfnnz52c4lyl/bafkreiddgwqs4k6dyk6dsanb2635o6qw4zzypyrysglxw7wpvg5v7ldyoy@webp/200x200`

Additional parameters for a proxied image can be specified by appending an `@` after the cid and providing a slash separated list from the following parameters:

* `<Width>x<Height>` - Resizes the image to the specified width & height.
* `<Format>` - Transforms the image to the requested format, only supports `PNG`, `GIF`, `WEBP`, and `JPEG` currently. Defaults to `Best` which picks the most suitable format for the requesting client.

## Raw Blob Proxy
`/blob/<did>/<cid>`

Just proxies the blob as is, without any attempts at transforming it iin any way.