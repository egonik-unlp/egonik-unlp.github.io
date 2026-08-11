/* Scroll-aware nav + progressive reveal motion. Content is visible by default;
   these only enhance when JS runs. */
(function () {
    "use strict";

    // Nav solidifies once the dark hero scrolls past the bar.
    var nav = document.querySelector(".site-nav");
    var hero = document.querySelector(".hero");
    if (nav && hero && "IntersectionObserver" in window) {
        var navH = 72;
        var navObs = new IntersectionObserver(
            function (entries) {
                nav.classList.toggle("is-solid", !entries[0].isIntersecting);
            },
            { rootMargin: "-" + navH + "px 0px 0px 0px", threshold: 0 }
        );
        navObs.observe(hero);
    } else if (nav) {
        nav.classList.add("is-solid");
    }

    // Reveal on scroll (staggered within a group). Queried live, because the work
    // index re-renders its rows when you switch between reading it by layer and
    // by question — rows inserted after load must be revealed too, or they stay
    // at opacity 0 forever.
    function pending() {
        return Array.prototype.slice.call(document.querySelectorAll("[data-reveal]"));
    }
    function revealAll() {
        pending().forEach(function (el) { el.classList.add("is-visible"); });
    }
    var items = pending();
    if (!items.length) return;

    if (!("IntersectionObserver" in window) || !("MutationObserver" in window)) {
        revealAll();
        return;
    }

    var obs = new IntersectionObserver(
        function (entries, o) {
            entries.forEach(function (e) {
                if (!e.isIntersecting) return;
                var el = e.target;
                var delay = parseInt(el.getAttribute("data-reveal-delay") || "0", 10);
                el.style.transitionDelay = delay + "ms";
                el.classList.add("is-visible");
                o.unobserve(el);
            });
        },
        { rootMargin: "0px 0px -8% 0px", threshold: 0.08 }
    );
    function watch(el) {
        if (!el.classList.contains("is-visible")) obs.observe(el);
    }
    items.forEach(watch);

    // Anything added later — a re-rendered row — gets observed the same way.
    new MutationObserver(function (records) {
        records.forEach(function (record) {
            Array.prototype.forEach.call(record.addedNodes, function (node) {
                if (node.nodeType !== 1) return;
                if (node.hasAttribute("data-reveal")) watch(node);
                Array.prototype.forEach.call(node.querySelectorAll("[data-reveal]"), watch);
            });
        });
    }).observe(document.body, { childList: true, subtree: true });

    // Safety net: never leave content hidden.
    window.addEventListener("load", function () {
        window.setTimeout(revealAll, 2500);
    });
})();
