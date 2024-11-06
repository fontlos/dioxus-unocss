import { FileSystemIconLoader } from '@iconify/utils/lib/loader/node-loaders'
import {
    defineConfig,
    // Dioxus not support empty attribute
    // presetAttributify,
    presetIcons,
    presetUno,
    transformerDirectives,
    transformerVariantGroup,
} from 'unocss'

export default defineConfig({
    content: {
        pipeline: {
            include: [
                // include rs files
                'src/**/*.rs',
            ],
            // exclude files
            exclude: []
            },
        },
    presets: [
        presetUno(),
        // presetAttributify(),
        presetIcons({
            collections: {
                icon: FileSystemIconLoader(
                    './assets/svg',
                    svg => svg.replace(/#000000/, 'currentColor'),
                ),
            },
            scale: 1,
            warn: true,
        }),
    ],
    transformers: [
        transformerDirectives(),
        transformerVariantGroup(),
    ],
})