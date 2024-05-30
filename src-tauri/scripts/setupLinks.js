// 在新窗口打开a标签
function setupLinks(node) {
    if (node.nodeName.toLowerCase() === 'a' && node.getAttribute("target") === "_blank") {
        node.onclick = function (event) {
            event.preventDefault();
            event.stopPropagation();
            window.open(node.href, "_blank")
            return false;
        };

    }

    if (node.childNodes.length > 0) {
        node.childNodes.forEach((childNode) => {
            setupLinks(childNode);
        });
    }
}

const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
        mutation.addedNodes.forEach((node) => {
            setupLinks(node);
        });
    });
});

const target = window.document;
const config = { childList: true, subtree: true };
observer.observe(target, config);
