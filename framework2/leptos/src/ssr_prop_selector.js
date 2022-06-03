function selectEl(root, path) {
	if (path.length == 0) {
		return root;
	} else if (path.length == 1) {
		return root.querySelector(`[data-leptos-hydrate="${path[0]}"]`);
	} else {
		const nextChild = path[0],
			nextChildEl = root.querySelector(`[data-leptos-hydrate="${nextChild}"]`);
		return selectEl(nextChildEl.shadowRoot, path.slice(1));
	}
}