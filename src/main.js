window.addEventListener('keydown', evt => {
    if (evt.key === 'ArrowLeft' || evt.key === "Prev" || evt.key === "PageUp") {
		window.location.pathname = Number(window.location.pathname.slice(1)) - 1;
    } else if (evt.key === 'ArrowRight' || evt.key === "Next" || evt.key === "PageDown") {
		window.location.pathname = Number(window.location.pathname.slice(1)) + 1;
    }
}, false);
