const plugin = require('tailwindcss/plugin');

export default plugin(({ matchVariant }) => {
  matchVariant(
    'last-in-seq',
    (value) => {
      // `value` is whatever's inside the brackets, e.g. "button"
      return [
        `&:last-child`,
        `&:not(:has(+ ${value}))`
      ]// or whatever selector logic you need
    }
  )
})
