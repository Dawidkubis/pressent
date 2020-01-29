window.addEventListener('keydown', evt => {
    if (evt.key === 'ArrowLeft') {
		window.location.pathname = Number(window.location.pathname.slice(1)) - 1;
    } else if (evt.key === 'ArrowRight') {
		window.location.pathname = Number(window.location.pathname.slice(1)) + 1;
    }
}, false);
