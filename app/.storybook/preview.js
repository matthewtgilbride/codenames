import {addDecorator} from "@storybook/react";
import {GlobalStyle, Layout} from "../design/layout";

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
}

addDecorator(story => (
    <>
      <GlobalStyle />
      <Layout>
        {story()}
      </Layout>
    </>
))