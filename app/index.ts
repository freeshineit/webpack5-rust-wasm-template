import './index.scss';
import type { ArithmeticOperation as ArithmeticOperationType } from '@rust';

/**
 * WASM Module Interface
 */
interface WasmModule {
  ArithmeticOperation: typeof ArithmeticOperationType;
  default: (module_or_path?: InitInput) => Promise<InitOutput>;
}

type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;
type InitOutput = any;

/**
 * Arithmetic Operations Demo
 */
class ArithmeticDemo {
  private module: WasmModule | null = null;

  /**
   * Initialize and load WASM module
   */
  async init(): Promise<void> {
    try {
      console.log('🦀 Loading Rust WASM module...');

      // Dynamic import with proper typing
      this.module = (await import('@rust')) as unknown as WasmModule;

      // Initialize WASM module (required for --target web)
      await this.module.default();

      console.log('✅ WASM module loaded successfully');

      this.runDemo();
    } catch (error) {
      console.error('❌ Failed to load WASM module:', error);
      this.displayError('Failed to initialize WASM module');
    }
  }

  /**
   * Run arithmetic operations demo
   */
  private runDemo(): void {
    if (!this.module) {
      console.error('Module not initialized');
      return;
    }

    const { ArithmeticOperation } = this.module;

    console.log('\n🧮 Arithmetic Operations Demo:');
    console.log('━'.repeat(50));

    // Test operations with error handling
    this.testOperation('Addition', () => ArithmeticOperation.addition(1, 5));
    this.testOperation('Subtraction', () => ArithmeticOperation.subtraction(10, 9));
    this.testOperation('Multiplication', () => ArithmeticOperation.multiplication(5, 2));
    this.testOperation('Division', () => ArithmeticOperation.division(10, 2));
    this.testOperation('Modulo', () => ArithmeticOperation.modulo(10, 3));
    this.testOperation('Power', () => ArithmeticOperation.power(2, 3));

    // Test error cases
    console.log('\n⚠️  Error Handling Tests:');
    console.log('━'.repeat(50));
    this.testOperation('Division by Zero', () => ArithmeticOperation.division(10, 0));
    this.testOperation('Modulo by Zero', () => ArithmeticOperation.modulo(10, 0));

    this.updateUI();
  }

  /**
   * Test a single operation with error handling
   */
  private testOperation(name: string, operation: () => number): void {
    try {
      const result = operation();
      console.log(`✓ ${name}: ${result}`);
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error);
      console.error(`✗ ${name}: ${errorMsg}`);
    }
  }

  /**
   * Update UI with demo information
   */
  private updateUI(): void {
    const container = document.querySelector('.container');
    if (container) {
      container.innerHTML = `
        <div class="content">
          <h1>Webpack5 + Rust + WASM</h1>
          <p class="subtitle">High-Performance Web Computing</p>
          <div class="info">
            <p>✅ WASM Module Loaded</p>
            <p>🦀 Rust Arithmetic Operations</p>
            <p>📊 Check Console for Results</p>
          </div>
        </div>
      `;
    }
  }

  /**
   * Display error message
   */
  private displayError(message: string): void {
    const container = document.querySelector('.container');
    if (container) {
      container.innerHTML = `
        <div class="error">
          <h1>Error</h1>
          <p>${message}</p>
        </div>
      `;
    }
  }
}

// Initialize the demo
const demo = new ArithmeticDemo();
demo.init().catch(console.error);
