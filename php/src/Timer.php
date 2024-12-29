<?php
namespace openiap;
class Timer
{
    private $callback;
    private $interval;
    private $running = false;

    public function __construct(callable $callback, int $interval)
    {
        $this->callback = $callback;
        $this->interval = $interval;
    }

    public function start(): void
    {
        $pid = pcntl_fork();

        if ($pid === -1) {
            die("Could not fork process.\n");
        } elseif ($pid === 0) {
            // Child process runs the timer
            $this->running = true;
            while ($this->running) {
                sleep($this->interval);
                call_user_func($this->callback);
            }
            exit(0); // Exit child process
        } else {
            // Parent process continues execution
            echo "Timer started in the background (PID: $pid).\n";
        }
    }

    public function stop(): void
    {
        $this->running = false;
    }
}
?>