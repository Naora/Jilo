import { LitElement } from "lit";
import { Pages } from "./models";
/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
export declare class JiloPageList extends LitElement {
    static styles: import("lit").CSSResult;
    pages: Pages[];
    isLoading: boolean;
    connectedCallback(): void;
    fetchAllPages(): Promise<void>;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        "jilo-page-list": JiloPageList;
    }
}
