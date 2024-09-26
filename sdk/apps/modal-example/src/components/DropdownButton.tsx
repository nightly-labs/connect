import { createSignal } from 'solid-js'

interface IDropdownProps<T extends { name: string }> {
  options: T[]
  label: string
  onClickOption: (item: T) => void
}

export function DropdownButton<T extends { name: string }>({
  options,
  label,
  onClickOption
}: IDropdownProps<T>) {
  const [isExpanded, setIsExpanded] = createSignal<boolean>(false)

  return (
    <div style={{ display: 'flex', 'flex-direction': 'column', 'align-items': 'center' }}>
      <button
        style={{ position: 'relative' }}
        onClick={() => {
          setIsExpanded(!isExpanded())
        }}>
        {label}
        <svg
          style={{
            position: 'absolute',
            right: '16px',
            top: '20px',
            transform: `rotate(${isExpanded() ? '180deg' : '0deg'})`
          }}
          width="16"
          height="10"
          viewBox="0 0 12 8"
          fill="none"
          xmlns="http://www.w3.org/2000/svg">
          <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M6 7.68555L0.219672 1.90522C-0.0732217 1.61232 -0.0732216 1.13745 0.219671 0.844557C0.512564 0.551664 0.987438 0.551664 1.28033 0.844557L6 5.56423L10.7197 0.844558C11.0126 0.551665 11.4874 0.551665 11.7803 0.844558C12.0732 1.13745 12.0732 1.61232 11.7803 1.90522L6 7.68555Z"
            fill="#FFFFFF"
          />
        </svg>
      </button>
      <div class="dropdown-list" style={{ 'max-height': isExpanded() ? '1000px' : '0px' }}>
        {options.map((item) => (
          <div onClick={() => onClickOption(item)} class="dropdown-item">
            {item.name}
          </div>
        ))}
      </div>
    </div>
  )
}
