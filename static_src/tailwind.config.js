module.exports = {
    future: {
        removeDeprecatedGapUtilities: true,
        purgeLayersByDefault: true,
    },
    purge: {
        enabled: true,
        mode: 'all',
        content: [
            '../templates/*.html',
        ]
    },
    theme: {
        extend: {},
    },
    variants: {},
    plugins: [],
};