export type ValidationErrorMetadata = {
  code: string;
  path: string;
  title: string;
  detail?: string;
};

export type ParseErrorLocation = {
  index: number;
  line: number;
  column: number;
};

export type ParseErrorMetadata = {
  code: string;
  title: string;
  detail: string;
  location?: ParseErrorLocation;
};

export type ValidationError =
  | { meta: ValidationErrorMetadata; states?: ValidationState[] }
  | { meta: ParseErrorMetadata };

export type ValidationState = {
  errors: ValidationError[];
};

export function validateAction(src: string): ValidationState;
export function validateWorkflow(src: string): ValidationState;
