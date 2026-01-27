(() => {
  function getDocLang() {
    return (document.documentElement.getAttribute("lang") || "").toLowerCase();
  }

  function main() {
    const rightButtons = document.querySelector(".right-buttons");
    if (!rightButtons) return;

    // mdBook defines this on every page.
    // eslint-disable-next-line no-undef
    const root = typeof path_to_root === "string" ? path_to_root : "";

    const lang = getDocLang();
    const isEn = lang === "en";
    const isZh = lang === "zh-cn";

    const wrapper = document.createElement("span");
    wrapper.className = "lang-select";

    const select = document.createElement("select");
    // Keep labels ASCII-only to avoid mixing languages in either locale.
    select.setAttribute("aria-label", "lang");

    const optEn = document.createElement("option");
    optEn.value = "en";
    optEn.textContent = "en";

    const optZh = document.createElement("option");
    optZh.value = "zh-CN";
    optZh.textContent = "zh-CN";

    select.appendChild(optEn);
    select.appendChild(optZh);
    select.value = isZh ? "zh-CN" : "en";

    select.addEventListener("change", () => {
      const v = select.value;
      const currentUrl = new URL(window.location.href);
      const bookRootUrl = new URL(root, currentUrl);
      const siteRootUrl = isZh ? new URL("../", bookRootUrl) : bookRootUrl;
      const relPath = currentUrl.pathname.startsWith(bookRootUrl.pathname)
        ? currentUrl.pathname.slice(bookRootUrl.pathname.length)
        : "";

      if (v === "zh-CN") {
        window.location.href = new URL(`zh-CN/${relPath}`, siteRootUrl).toString();
      } else {
        window.location.href = new URL(relPath, siteRootUrl).toString();
      }
    });

    wrapper.appendChild(select);
    rightButtons.prepend(wrapper);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", main, { once: true });
  } else {
    main();
  }
})();
