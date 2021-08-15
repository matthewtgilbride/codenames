import {addDecorator} from "@storybook/react";
import {GlobalStyle, Layout} from "../design/layout";
import {Global} from "@emotion/react";

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
      <Global styles={GlobalStyle} />
      <Layout>
        {story()}
      </Layout>
    </>
))