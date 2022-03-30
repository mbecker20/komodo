/* CSS from https://loading.io/css/ */

export function spinnerCss(scale = 0.5) {
  return `
    .Spinner {
      display: inline-block;
      position: relative;
      width: ${80 * scale}px;
      height: ${80 * scale}px;
    }
    .Spinner div {
      box-sizing: border-box;
      display: block;
      position: absolute;
      width: ${64 * scale}px;
      height: ${64 * scale}px;
      margin: ${8 * scale}px;
      border: ${8 * scale}px solid #fff;
      border-radius: 50%;
      animation: spinner 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
      border-color: #fff transparent transparent transparent;
    }
    .Spinner div:nth-child(1) {
      animation-delay: -0.45s;
    }
    .Spinner div:nth-child(2) {
      animation-delay: -0.3s;
    }
    .Spinner div:nth-child(3) {
      animation-delay: -0.15s;
    }
    @keyframes spinner {
      0% {
        transform: rotate(0deg);
      }
      100% {
        transform: rotate(360deg);
      }
    }
  `;
}

export function sonarCss(scale = 0.5) {
	return `
		.Sonar {
			display: inline-block;
			position: relative;
			width: ${80 * scale}px;
      height: ${80 * scale}px;
		}
		.Sonar div {
			position: absolute;
			border: ${4 * scale}px solid #fff;
			opacity: 1;
			border-radius: 50%;
			animation: sonar 1s cubic-bezier(0, 0.2, 0.8, 1) infinite;
		}
		.Sonar div:nth-child(2) {
			animation-delay: -0.5s;
		}
		@keyframes sonar {
			0% {
				top: ${36 * scale}px;
				left: ${36 * scale}px;
				width: 0;
				height: 0;
				opacity: 1;
			}
			100% {
				top: 0px;
				left: 0px;
				width: ${72 * scale}px;
				height: ${72 * scale}px;
				opacity: 0;
			}
		}
	`;
}

export function threeDotsCss(scale = 0.5) {
	return `
		.ThreeDot {
			display: inline-block;
			position: relative;
			width: ${80 * scale}px;
      height: ${80 * scale}px;
		}
		.ThreeDot div {
			position: absolute;
			top: ${33 * scale}px;
			width: ${13 * scale}px;
			height: ${13 * scale}px;
			border-radius: 50%;
			background: #fff;
			animation-timing-function: cubic-bezier(0, 1, 1, 0);
		}
		.ThreeDot div:nth-child(1) {
			left: ${8 * scale}px;
			animation: three-dot1 0.6s infinite;
		}
		.ThreeDot div:nth-child(2) {
			left: ${8 * scale}px;
			animation: three-dot2 0.6s infinite;
		}
		.ThreeDot div:nth-child(3) {
			left: ${32 * scale}px;
			animation: three-dot2 0.6s infinite;
		}
		.ThreeDot div:nth-child(4) {
			left: ${56 * scale}px;
			animation: three-dot3 0.6s infinite;
		}
		@keyframes three-dot1 {
			0% {
				transform: scale(0);
			}
			100% {
				transform: scale(1);
			}
		}
		@keyframes three-dot2 {
			0% {
				transform: translate(0, 0);
			}
			100% {
				transform: translate(${24 * scale}px, 0);
			}
		}
		@keyframes three-dot3 {
			0% {
				transform: scale(1);
			}
			100% {
				transform: scale(0);
			}
		}
	`;
}
