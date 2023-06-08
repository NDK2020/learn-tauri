import { defineConfig, presetUno, transformerVariantGroup } from 'unocss'
import presetRemToPx from '@unocss/preset-rem-to-px'

export default defineConfig({
  content: {
    pipeline: {
      // include: [
      //   // the default
      //   /\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html)($|\?)/,
      //   // include js/ts files
      //   'src/**/*.{js,ts}',
      // ],
      // exclude files
      exclude: ['node_modules', '.git', 'dist'],
    }
  },
  presets: [presetUno(), presetRemToPx({ baseFontSize: 4 })],
  transformers: [transformerVariantGroup()],
  shortcuts: {
    'flex-col': 'flex flex-col',
    'flex-row': 'flex flex-row',
    'flex-center': 'flex justify-center items-center',
    'flex-start': 'flex justify-start items-center',
    'flex-end': 'flex justify-end items-center',
    'flex-between': 'flex justify-between items-center',
    'flex-evenly': 'flex justify-evenly items-center',
    'flex-around': 'flex justify-around items-center',
  },
})
