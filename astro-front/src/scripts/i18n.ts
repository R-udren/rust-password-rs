(() => {
  const k = 'lang';
  const s = localStorage.getItem(k);
  const ru = navigator.language.startsWith('ru');
  const onRu = location.pathname.startsWith('/ru');
  if (!s) {
    localStorage.setItem(k, ru ? 'ru' : 'en');
    if (ru && !onRu) location.replace('/ru' + location.pathname);
  }
})();
