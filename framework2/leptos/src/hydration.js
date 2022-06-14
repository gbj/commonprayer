const hasLoaded = {};

export function observe_custom_elements(root) {
	const observer = new IntersectionObserver(async (entries) => {
		for(const entry of entries) {
			if (entry.intersectionRatio > 0) {
				const tagName = entry.target.tagName.toLowerCase();
				if(!hasLoaded[tagName]) {
					hasLoaded[tagName] = true;
					const moduleName = tagName.replace("l-", "").replace(/\-/g, "_");
					import(`/client/${moduleName}/pkg/${moduleName}_wc.js`).then(async pkg => {
						await pkg.default();
						pkg.define_custom_elements();
					});
				}
	
				// no longer need to observe this element for changes
				observer.unobserve(entry.target);
			}
		}
	});
	
	// start observing
	for(const el of root.querySelectorAll("[data-leptos-hydrate]")) {
		observer.observe(el);
	}
}