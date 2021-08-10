export function getAssetAsString(assetName: string, customFetch = fetch) {
	return customFetch(`/${assetName}`).then((r) => r.text());
}

export function getBlogPost(postName: string, customFetch = fetch) {
	return getAssetAsString(`_blog/${postName}`, customFetch)
}
