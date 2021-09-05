import {addDecorator} from "@storybook/react";
import {Layout} from "../design/layout";
import {Global} from "@emotion/react";
import {GlobalStyle} from "../design/GlobalStyle";

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