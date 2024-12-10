# ATProto Proxy
This is a WIP service for proxying ATProto blobs and transforming them for the requesting client. Currently this only supports proxying images.

## Image Proxy
`/img/<did>/<cid>@<parameters?>`

ex. `https://example.com/img/did:plc:ui5pgpumwvufhfnnz52c4lyl/bafkreiddgwqs4k6dyk6dsanb2635o6qw4zzypyrysglxw7wpvg5v7ldyoy@webp/200x200`

Additional parameters for a proxied image can be specified by appending an `@` after the cid and providing a slash separated list from the following parameters:

* `<Width>x<Height>` - Resizes the image to the specified width & height.
* `<Format>` - Transforms the image to the requested format, only supports `PNG`, `GIF`, `WEBP`, and `JPEG` currently. Defaults to `Best` which picks the most suitable format for the requesting client.