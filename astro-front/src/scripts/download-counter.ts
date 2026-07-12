const OWNER = "R-udren";
const REPO = "rust-password-rs";
const API = `https://api.github.com/repos/${OWNER}/${REPO}/releases/latest`;

interface Asset {
  name: string;
  download_count: number;
}

interface Release {
  assets: Asset[];
}

function format(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
  return n.toLocaleString();
}

async function init() {
  const els = document.querySelectorAll<HTMLElement>("[data-dl-count]");
  if (!els.length) return;

  try {
    const res = await fetch(API);
    if (!res.ok) return;
    const data: Release = await res.json();
    const asset = data.assets.find((a) => a.name === "rust-password.exe");
    const count = asset?.download_count;
    if (!count) return;
    const formatted = format(count);
    for (const el of els) {
      el.textContent = formatted;
    }
  } catch {
    // silent fail — keeps "—" placeholder
  }
}

init();
