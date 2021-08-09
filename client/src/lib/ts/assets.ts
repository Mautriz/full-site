export function getAssetAsString(assetName: string) {
	return fetch(`/static/${assetName}`).then((r) => r.text());
}
