import type { GridCommand, GridModel } from "./commands";

/** Standard linear undo/redo stack: `do` clears any redo history, since once
 * a new edit branches off from a past state the old redo branch is stale. */
export class CommandStack {
  #undo: GridCommand[] = [];
  #redo: GridCommand[] = [];

  get canUndo(): boolean {
    return this.#undo.length > 0;
  }

  get canRedo(): boolean {
    return this.#redo.length > 0;
  }

  do(model: GridModel, command: GridCommand): void {
    command.do(model);
    this.#undo.push(command);
    this.#redo = [];
  }

  undo(model: GridModel): void {
    const command = this.#undo.pop();
    if (!command) return;
    command.undo(model);
    this.#redo.push(command);
  }

  redo(model: GridModel): void {
    const command = this.#redo.pop();
    if (!command) return;
    command.do(model);
    this.#undo.push(command);
  }
}
