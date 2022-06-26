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
  @state() newPageName: string = "";

  connectedCallback(): void {
    super.connectedCallback();

    this.fetchAllPages();
  }

  async fetchAllPages() {
    try {
      const response = await fetch("/api/v1/pages");
      const json = await response.json();
      this.pages = json;
      this.isLoading = false;
    } catch (error) {
      console.error(error);
    }
  }

  private async _onDelete(event: MouseEvent) {
    const id = (event.target as Element).getAttribute("data-page")
    await fetch(`/api/v1/pages/${id}`, {method: "DELETE"})
    await this.fetchAllPages()
  }

  private async _onCreate() {
    const body = JSON.stringify({name: this.newPageName, template: "/pages/article"})
    const headers = new Headers()
    headers.append("content-type", "text/json")
    await fetch(`/api/v1/pages`, {method: "POST", body, headers})
    this.newPageName = "";
    await this.fetchAllPages()
  }


  private _onInput(event: InputEvent) {
    this.newPageName = (event.target as HTMLInputElement).value
  }

  render() {
    const loadingScreen = html`<span>Loading...</span>`;

    const pageFetched = html`
      <div>
        <input type="text" @input=${this._onInput} .value=${this.newPageName} />
        <button @click=${this._onCreate}>Create</button>
        <ul>
          ${repeat(
            this.pages,
            (p) => p.id,
            (page, _) => html`<li><a href="/pages/${page.id}">${page.name}</a> <button data-page=${page.id} @click=${this._onDelete} >Delete</button></li>`
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
