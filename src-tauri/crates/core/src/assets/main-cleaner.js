function waitForElm(selector) {
  return new Promise((resolve) => {
    if (document.querySelector(selector)) {
      return resolve(document.querySelector(selector));
    }

    const observer = new MutationObserver(() => {
      if (document.querySelector(selector)) {
        observer.disconnect();
        resolve(document.querySelector(selector));
      }
    });

    // If you get "parameter 1 is not of type 'Node'" error, see https://stackoverflow.com/a/77855838/492336
    observer.observe(document.documentElement, {
      childList: true,
      subtree: true,
    });
  });
}

function infiniteRemove(selector) {
  waitForElm(selector).then((element) => {
    element.remove();
    infiniteRemove(selector);
  });
}

let timeout = 1;
function removeButtons() {
  if (!document.documentElement) {
    window.setTimeout(removeButtons, Math.round(timeout));
    timeout *= 1.5;
    return;
  }
  timeout = 1;
  infiniteRemove("ul.top-actions>li");
  infiniteRemove("div.divider");
  infiniteRemove("div.split-button");
}

removeButtons();
