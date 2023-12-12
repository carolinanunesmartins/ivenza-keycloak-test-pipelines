// TODO: get the config, or its properties, from a shared lib
const brandColors = {
    orange: {
        100: '#FDF2E5',
        500: '#F07D00',
        600: '#D97100',
        900: '#301900',
    },
    blue: {
        100: '#E5EDF5',
        200: '#C9DDF4',
        500: '#024D9E',
        600: '#013165',
        900: '#000F20',
    },
    red: {
        100: '#FDEBEB',
        500: '#F23A3C',
        600: '#C22E30',
        900: '#300C0C',
    },
    green: {
        100: '#EDF9F3',
        500: '#48BD84',
        600: '#3A976A',
        900: '#0E261A',
    },
    gray: {
        900: '#1F3333',
        800: '#445656',
        700: '#5F7070',
        600: '#848F8F',
        500: '#A6ACAC',
        400: '#CDCFCF',
        300: '#DDE1E1',
        250: '#E6E6E6',
        200: '#ECEFEF',
        100: '#F2F5F5',
        50: '#F7FAFA',
    },
    white: '#FFFFFF',
    black: '#000000',
};

const brandSemanticColors = {
    primary: brandColors.orange,
    secondary: brandColors.blue,
    success: brandColors.green,
    neutral: brandColors.gray,
    warning: brandColors.orange,
    error: brandColors.red,
    informative: brandColors.blue, // should use informative if it is not a primary action, but rather to inform someone
};

/** @type {import('tailwindcss').Config} */
// eslint-disable-next-line no-undef
module.exports = {
    content: ['./**/*.ftl'],
    theme: {
        extend: {
            screens: {
                xs: '468px',
            },
            fontFamily: {
                roihu: ['Roihu', 'sans-serif'],
            },
            colors: {
                brand: {
                    ...brandSemanticColors,
                    gray: brandColors.gray,
                    black: brandColors.black,
                    white: brandColors.white,
                    green: brandColors.green,
                    blue: brandColors.blue,
                    red: brandColors.red,
                },
                'brand-orange': brandColors.orange['500'],
                'brand-orange-hover': brandColors.orange['600'],
                'brand-green': brandColors.green['500'],
                'brand-dark-green': brandColors.green['900'],
                'brand-blue': brandColors.blue['500'],
                'brand-blue-hover': brandColors.blue['600'],
            },
            // boxShadow: {
            //     accessory: 'inset 0 0 0 0.1rem #002856',
            //     'inner-orange': 'inset 0 -1px 0px 0px ' + brandColors.orange['500'],
            //     'inner-orange-thick': 'inset 0 -2px 0px 0px ' + brandColors.orange['500'],
            //     'inner-green': 'inset 0 -1px 0px 0px ' + brandColors.green['500'],
            //     'inner-green-thick': 'inset 0 -2px 0px 0px ' + brandColors.green['500'],
            //     'inner-gray': 'inset 0 -1px 0px 0px ' + brandColors.gray['500'],
            //     'inner-gray-thick': 'inset 0 -2px 0px 0px ' + brandColors.gray['500'],
            //     'inner-black': 'inset 0 -1px 0px 0px ' + brandColors.gray['900'],
            //     'inner-black-thick': 'inset 0 -2px 0px 0px ' + brandColors.gray['900'],
            //     'inner-gray-200': 'inset 0 -1px 0px 0px ' + brandColors.gray['200'],
            //     'inner-blue': 'inset 0 -1px 0px 0px ' + brandColors.blue['500'],
            //     'inner-blue-thick': 'inset 0 -2px 0px 0px ' + brandColors.blue['500'],
            //     'outer-brand-neutral-300': '0 0 0 3px ' + brandColors.gray['300'],
            //     'outer-brand-success-500': '0 0 0 3px ' + brandColors.green['500'],
            //     'inner-left-blue': 'inset  4px 0 0 0 ' + brandColors.blue['600'],
            //     'inner-left-orange': 'inset 4px 0 0 0 ' + brandColors.orange['600'],
            //     'inner-left-green': 'inset 4px 0 0 0 ' + brandColors.green['600'],
            //     'top-md': '0 -2px 4px -2px rgb(0 0 0 / 0.1), 0 -4px 8px -2px rgb(0 0 0 / 0.1)',
            //     stepper: '0 9999px 0px 9999px white',
            // },
            borderWidth: {
                3: '3px',
            },
            fontSize: {
                xxs: '0.625rem',
                'heading-2xl': '3.125rem',
                'heading-xl': '1.875rem',
                'heading-2xl-mobile': '2rem',
                'heading-xl-mobile': '1.75rem',
                'heading-lg': '1.5rem',
                'heading-md': '1.25rem',
                'heading-sm': '1.125rem',
            },
            minWidth: {
                '1/2': '50%',
            },
            spacing: {
                128: '32rem',
            },
            gridTemplateColumns: {
                minContentAuto: 'min-content auto',
            },
            transitionProperty: {
                width: 'width',
            },
            width: {
                29: '7.1rem',
            },
        },
    },
    plugins: [],
};
