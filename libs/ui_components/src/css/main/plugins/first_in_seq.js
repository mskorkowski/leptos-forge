const plugin = require('tailwindcss/plugin');

export default plugin(({ matchVariant }) => {
  matchVariant(
    'first-in-seq',
    (value) => {
      // `value` is whatever's inside the brackets, e.g. "button"
      return [
        `&:first-child`,
        `:not(${value}) + &`
      ]// or whatever selector logic you need
    }
  )
})
