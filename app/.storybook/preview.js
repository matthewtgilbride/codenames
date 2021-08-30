import {addDecorator} from "@storybook/react";
import {Layout} from "../design/layout";
import {GlobalStyle} from "../pages/_app";
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