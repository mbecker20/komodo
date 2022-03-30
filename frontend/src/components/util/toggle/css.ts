export function toggleCss(scale = 0.75) {
	return `
		.toggle {
			position: relative;
			display: inline-block;
			width: ${60 * scale}px;
			height: ${34 * scale}px;
		}

		/* Hide default HTML checkbox */
		.toggle input {
			opacity: 0;
			width: 0;
			height: 0;
		}

		/* The slider */
		.slider {
			position: absolute;
			cursor: pointer;
			top: 0;
			left: 0;
			right: 0;
			bottom: 0;
			background-color: #ccc;
			-webkit-transition: .4s;
			transition: .4s;
		}

		.slider:before {
			position: absolute;
			content: "";
			height: 26px;
			width: 26px;
			left: 4px;
			bottom: 4px;
			background-color: white;
			-webkit-transition: .4s;
			transition: .4s;
		}

		input:checked + .slider {
			background-color: #2196F3;
		}

		input:focus + .slider {
			box-shadow: 0 0 1px #2196F3;
		}

		input:checked + .slider:before {
			-webkit-transform: translateX(${26 * scale}px);
			-ms-transform: translateX(${26 * scale}px);
			transform: translateX(${26 * scale}px);
		}

		/* Rounded sliders */
		.slider.round {
			border-radius: ${30 * scale}px;
		}

		.slider.round:before {
			border-radius: 50%;
		}
	`;
}