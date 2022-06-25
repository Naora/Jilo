import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import { repeat } from "lit/directives/repeat.js";
import { Pages } from "./models";

/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
@customElement("jilo-page-list")
export class JiloPageList extends LitElement {
  static styles = css`
    :host {
      display: block;
      border: solid 1px gray;
      padding: 16px;
      max-width: 800px;
    }
  `;

  @state() pages: Pages[] = [];
  @state() isLoading: boolean = true;

  connectedCallback(): void {
    super.connectedCallback();

    this.fetchAllPages();
  }

  async fetchAllPages() {
    const response = await fetch("/api/v1/pages");
    const json = await response.json();
    this.pages = json.data;
    this.isLoading = false;
  }

  render() {
    const loadingScreen = html`<span>Loading...</span>`;

    const pageFetched = html`
      <div>
        <ul>
          ${repeat(
            this.pages,
            (p) => p.id,
            (page, _) => html` <li>${page.name}</li> `
          )}
        </ul>
      </div>
    `;

    return html` <div>${this.isLoading ? loadingScreen : pageFetched}</div> `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "jilo-page-list": JiloPageList;
  }
}
