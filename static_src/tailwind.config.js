module.exports = {
    future: {
        removeDeprecatedGapUtilities: true,
        purgeLayersByDefault: true,
    },
    purge: {
        enabled: false,
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