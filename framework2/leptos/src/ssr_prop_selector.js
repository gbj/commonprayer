function selectEl(root, path) {
	if (path.length == 0) {
		return root;
	} else if (path.length == 1) {
		return root.querySelector(`[data-leptos-hydrate="${path[0]}"]`);
	} else {
		const nextChild = path[0],
			nextChildEl = root.querySelector(`[data-leptos-hydrate="${nextChild}"]`);
		console.log("nextChild", nextChild, "nextChildEl", nextChildEl)
		return selectEl(nextChildEl ? nextChildEl.shadowRoot : document, path.slice(1));
	}
}