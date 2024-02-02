'use client';

import { Box, Button, Grid, TextField } from '@mui/material';
import { useState } from 'react';

// import { ExecuteForm } from '@/components/form';
import { Toaster } from '@/components/notifications';
import { useLocale, useToastLimit } from '@/hooks';
import { command } from '@/tauri_cmd';

export default function App() {
  useLocale();
  useToastLimit(1);
  const [program, setProgram] = useState('');
  const [args, setArgs] = useState('');

  return (
    <>
      <Box
        component="main"
        sx={{
          display: 'grid',
          minHeight: 'calc(100vh - 56px)',
          placeContent: 'center',
          placeItems: 'center',
          width: '100%',
        }}
      >
        <Grid container spacing={2}>
          <Grid xs={6}>
            <TextField
              id="outlined-basic"
              label="Program"
              placeholder="wsl.exe"
              variant="outlined"
              onChange={function (event) {
                setProgram(event.target.value);
              }}
            >
              {program}
            </TextField>
          </Grid>

          <Grid xs={6}>
            <TextField
              id="outlined-basic"
              label="Arguments"
              placeholder="--exec ls"
              variant="outlined"
              onChange={function (event) {
                setArgs(event.target.value);
              }}
            >
              {args}
            </TextField>
          </Grid>
        </Grid>

        <Button
          variant="contained"
          onClick={async function () {
            await command(program, args.split(' '));
          }}
        >
          Execute
        </Button>
        {/* <ExecuteForm /> */}
      </Box>
      <Toaster />
    </>
  );
}
