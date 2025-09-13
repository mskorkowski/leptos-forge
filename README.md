# Leptos Forge

Leptos Forge is the library which helps with creating and testing user interfaces written in Rust and [Leptos](https://leptos.dev/).

> **Warning**
>
> Leptos Forge is still at the early stages of development and is really bare-bone project. Many of the developer experience features
> are not yet implemented.

## Features

- **Trivial integration with Leptos** - You can use Leptos components directly in your Forge stories without any additional setup
- **Customizable** - It's easy to add new feature and customize almost any aspect of the resulting application to fit your needs
- **Descriptive** - Leptos Froge allows you to write your descriptions using Markdown
- **Rust API** - This project was created with programming Rust in mind to make it easy to write your stories

## Leptos Forge vs Storybook

[Storybook](https://storybook.js.org/) is a popular tool used to create frontend components in isolation and test them and was a big 
inspiration for Leptos Forge. However, Leptos Forge has some unique features that make it different from Storybook:

| Feature | Leptos Forge | Storybook |
|:--------|:------------:|:---------:|
| Programming language | Rust | JavaScript/TypeScript |
| Targeted frameworks | Leptos | React, Vue, Angular and many more |
| Usage | It's a library around which you write your stories | It's an application that manages your stories |
| Extending | Since you control the entire application, you can easily extend it to fit your needs | Storybook has a lot's of features and extensions to meet your needs but if you need something not on that list, it's not so easy to add it yourself |
| Look | Ugly, let's be frank | It's clean and nice to look at |
| Which should I use? | If you are using Rust and Leptos, you might want to consider Lepotos Forge. You must remember it's still a bleeding edge project. | If Storybook meets your needs, go there. It's mature, stable and supported by a lots of people |


## Version numbers and how stable it is?

For versions `1+` we will follow the `SemVer` specification but it's still a far away future. 

To make it a bit easier to play with the current version of the project we will use the convention similar to the `SemVer`, but breaking
changes instead of major version will update a minor version number and we will create a migration information. Nonbreaking changes will
update a patch version number. If we do a big change in the lib we will also bump the minor version.

In terms of stability I (Marek) can say that core should be rather stable. There are two reasons for this:

1. I'm using Leptos Forge for my private projects and it does most of the things I would like to do with Leptos Forge*
2. I'm human and rewriting all of the stories I've created for my projects won't be fun

*There is quite a lot of UX issues in Leptos Forge, which will be ironed out in the future but due to nature of the Leptos Forge
they are mostly isolated.

## Getting Started

TODO: Add getting started instructions here.

## License

Project is under MIT license. See [LICENSE](LICENSE) for details.

## Contributing

Create an issue where you will describe what and why would you like to change. We will iron it out together. All PR are welcome.

### Why Leptos Forge uses custom components

Leptos Forge is using internally the components defined in the `libs/ui_components` crate. We've created these components so you can use them in the code
related to your stories. This way Leptos Forge doesn't depend on any particular UI library, so you can use whatever you like without worrying about 
compatibility issues.