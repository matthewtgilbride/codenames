import {addDecorator} from "@storybook/react";
import {Layout} from "../design/layout";
import {Global} from "@emotion/react";
import {GlobalStyle} from "../design/GlobalStyle";
import {ApiContextProvider} from "../components/ApiContext";

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
        <ApiContextProvider baseUrl="http://localhost:8080">
          {story()}
        </ApiContextProvider>
      </Layout>
    </>
))