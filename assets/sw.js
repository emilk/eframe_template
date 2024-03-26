var cacheName = 'egui-template-pwa';
var filesToCache = [
  './',
  './index.html',
  './eframe_template.js',
  './eframe_template_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});

/* Clear cache on version updates */
self.addEventListener("activate", function(e) {
  e.waitUntil(
    caches.open(cacheName)
    .then(cache => {
      return cache.match("/version"); // Try to find the stored version in the cache
    })
    .then(response => {
      if (response) {
        return response.text();
      } else {
        return null; // No stored version found in the cache
      }
    })
    .then(storedVersion => {
      return fetch("/version")
        .then(response => response.text())
        .then(backendVersion => {
          if (backendVersion !== storedVersion) {
            return caches.keys().then(cacheNames => {
              return Promise.all(
                cacheNames.map(cacheName => {
                  return caches.delete(cacheName);
                })
              );
            });
          }
        });
    })
    .catch(error => {
      console.error("Error: ", error);
    })
  );
});
