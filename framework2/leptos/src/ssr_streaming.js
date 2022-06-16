function updateStreamedNode(idx) {
	console.log("updateStreamedNode", idx, document.querySelector(`[data-leptos-async-pending='${idx}']`), document.querySelector(`[data-leptos-async-ready='${idx}']`))
	const newEl = document.querySelector(`[data-leptos-async-ready='${idx}']`),
		oldEl = document.querySelector(`[data-leptos-async-pending='${idx}']`);
	oldEl.replaceWith(newEl);
}