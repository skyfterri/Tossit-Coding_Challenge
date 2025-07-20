import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config'; // This import is correct for app.config.ts
import { AppComponent } from './app/app'; // <--- CHANGE THIS LINE

bootstrapApplication(AppComponent, appConfig) // <--- CHANGE THIS LINE
  .catch((err) => console.error(err));