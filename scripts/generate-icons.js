// Generates the full app icon set from the vector sources in
// src-tauri/icons/sources/*.svg using @resvg/resvg-js + png-to-ico.
//
//   pnpm icons
//
// Edit the SVGs in src-tauri/icons/sources/, then re-run this script.
// Windows-only project: no .icns is produced (macOS uses it, we don't ship macOS).
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Resvg } from '@resvg/resvg-js';
import pngToIco from 'png-to-ico';

const ICONS = join(dirname(fileURLToPath(import.meta.url)), '..', 'src-tauri', 'icons');
const SOURCES = join(ICONS, 'sources');

/** Rasterize an SVG file to a square PNG buffer of the given width. */
function render(svgPath, size) {
  const resvg = new Resvg(readFileSync(svgPath), { fitTo: { mode: 'width', value: size } });
  return resvg.render().asPng();
}

function writePng(svgPath, outPath, size) {
  writeFileSync(outPath, render(svgPath, size));
  console.log(`  ${outPath}  (${size}x${size})`);
}

const refSvg = join(SOURCES, 'ref.svg');

console.log('Desktop icons (from sources/ref.svg):');
writePng(refSvg, join(ICONS, '32x32.png'), 32);
writePng(refSvg, join(ICONS, '64x64.png'), 64);
writePng(refSvg, join(ICONS, '128x128.png'), 128);
writePng(refSvg, join(ICONS, '128x128@2x.png'), 256);
writePng(refSvg, join(ICONS, 'icon.png'), 512);

console.log('Windows .ico (multi-resolution):');
const icoSizes = [16, 24, 32, 48, 64, 128, 256];
const icoBuffers = icoSizes.map((size) => render(refSvg, size));
const icoPath = join(ICONS, 'icon.ico');
writeFileSync(icoPath, await pngToIco(icoBuffers));
console.log(`  ${icoPath}  (${icoSizes.join(', ')})`);

console.log('Tray icons (32x32):');
writePng(refSvg, join(ICONS, 'tray', 'normal.png'), 32);
writePng(join(SOURCES, 'ref-unread.svg'), join(ICONS, 'tray', 'badge.png'), 32);
writePng(join(SOURCES, 'ref-offline.svg'), join(ICONS, 'tray', 'offline.png'), 32);

console.log('Done.');
