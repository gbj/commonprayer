const hasLoaded = {};

export function observe_custom_elements(root) {
	console.log("observe_custom_elements called");
	const observer = new IntersectionObserver(async (entries) => {
		for(const entry of entries) {
			if (entry.intersectionRatio > 0) {
				console.log("hydrating", entry.target.tagName);
				const tagName = entry.target.tagName.toLowerCase();
				if(!hasLoaded[tagName]) {
					hasLoaded[tagName] = true;
					const moduleName = tagName.replace("l-", "").replace(/\-/g, "_");
					import(`/client/${moduleName}/pkg/${moduleName}_wc.js`).then(async pkg => {
						await pkg.default();
						if (!customElements.get(tagName)) {
							pkg.define_custom_elements();
						}
					});
				}
	
				// no longer need to observe this element for changes
				observer.unobserve(entry.target);
			}
		}
	});
	
	// start observing
	for (const el of root.querySelectorAll("[data-leptos-hydrate]")) {
		console.log("observing", el);
		observer.observe(el);
	}
}