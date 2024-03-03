import cx from 'classnames';

interface Props {
  /** @default 24 */
  size?: number;
  className?: string;
}

export function ChevronDownIcon({ size = 24, className }: Props) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      strokeWidth={1.5}
      stroke="currentColor"
      style={{ width: size, height: size }}
      className={cx('inline-flex', className)}
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
    </svg>
  );
}
