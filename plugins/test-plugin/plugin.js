const supportedUnits = {
  km: 'Kilometer',
  m: 'Meter',
  cm: 'Centimeter',
  mm: 'Milimeter',
  in: 'Inch',
  ft: 'Foot',
  yd: 'Yard',
  mi: 'Mile',
  nmi: 'Nautical Mile'
}

const supportedUnitsPlural = {
  km: 'Kilometers',
  m: 'Meters',
  cm: 'Centimeters',
  mm: 'Milimeters',
  in: 'Inches',
  ft: 'Feet',
  yd: 'Yards',
  mi: 'Miles',
  nmi: 'Nautical Miles'
}

// We'll always go via meter as intermediary so that we can save space

/**
 * Contains conversions from anything to meters
 *
 * @var {Record<string, (value: number) => number>}
 */
const conversions2meter = {
  km: value => value * 1000,
  m: value => value,
  cm: value => value / 100,
  mm: value => value / 1000,
  in: value => value * 0.0254,
  ft: value => value * 0.3048,
  yd: value => value * 0.9144,
  mi: value => value * 1_609.344,
  nmi: value => value * 1852
}

/**
 * Contains conversions from meters to anything
 *
 * @var {Record<string, (number) => number>}
 */
const conversionsFromMeter = {
  km: value => value / 1000,
  m: value => value,
  cm: value => value * 100,
  mm: value => value * 1000,
  in: value => value / 0.0254,
  ft: value => value / 0.3048,
  yd: value => value / 0.9144,
  mi: value => value / 1_609.344,
  nmi: value => value / 1852
}

function updateResult () {
  const inValue = document.getElementById('in-value')
  const outValue = document.getElementById('out-value')
  const inUnit = document.getElementById('in-unit')
  const outUnit = document.getElementById('out-unit')

  console.log(inValue.value, inUnit.value, outUnit.value)

  const intermediary = conversions2meter[inUnit.value](parseFloat(inValue.value))
  let result = conversionsFromMeter[outUnit.value](intermediary)

  result = Math.round(result * 100) / 100

  if (result === 1.0) {
    outValue.textContent = `${result} ${supportedUnits[outUnit.value]}`
  } else {
    outValue.textContent = `${result} ${supportedUnitsPlural[outUnit.value]}`
  }

}

export default function plugin () {
  const dom = document.createElement('div')
  dom.style.padding = '20px'

  const inValue = document.createElement('input')
  inValue.id = 'in-value'
  inValue.type = 'number'

  inValue.style.width = '100%'
  inValue.style.display = 'block'
  inValue.style.fontSize = '200%'
  inValue.style.border = 'none'
  inValue.style.textAlign = 'center'
  inValue.autofocus = 'autofocus'

  inValue.addEventListener('keyup', updateResult)
  inValue.addEventListener('change', updateResult)
  dom.appendChild(inValue)

  const inUnit = document.createElement('select')
  inUnit.style.display = 'inline-block'
  inUnit.style.width = '50%'
  inUnit.style.margin = '10px 0'
  inUnit.style.border = 'none'
  inUnit.style.backgroundColor = 'transparent'
  inUnit.id = 'in-unit'
  inUnit.addEventListener('change', updateResult)
  for (const unit of Object.keys(supportedUnits)) {
    const option = document.createElement('option')
    option.value = unit
    option.textContent = supportedUnits[unit]
    inUnit.appendChild(option)
  }
  dom.appendChild(inUnit)

  const outUnit = document.createElement('select')
  outUnit.style.display = 'inline-block'
  outUnit.style.width = '50%'
  outUnit.style.margin = '10px 0'
  outUnit.style.border = 'none'
  outUnit.style.backgroundColor = 'transparent'
  outUnit.id = 'out-unit'
  outUnit.addEventListener('change', updateResult)
  for (const unit of Object.keys(supportedUnits)) {
    const option = document.createElement('option')
    option.value = unit
    option.textContent = supportedUnits[unit]
    outUnit.appendChild(option)
  }
  dom.appendChild(outUnit)

  const outValue = document.createElement('span')
  outValue.id = 'out-value'
  outValue.style.width = '100%'
  outValue.style.display = 'block'
  outValue.style.fontSize = '200%'
  outValue.style.textAlign = 'center'
  dom.appendChild(outValue)

  return { dom }
}
