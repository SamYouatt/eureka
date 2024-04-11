document.addEventListener("DOMContentLoaded", (_event) => {
    document.body.addEventListener("htmx:beforeSwap", function(hxEvent) {
        // We want to tell htmx to allow the body of 422 through, we will be giving html to render the required message to the user
        if (hxEvent.detail.xhr.status === 422) {
            hxEvent.detail.shouldSwap = true;
            hxEvent.detail.isError = false; // stops error appearing in console
        }
    });
})
