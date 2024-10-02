----------------------------------------------------------------------------------
-- Company:
-- Engineer:
--
-- Create Date: 09/25/2024 12:49:31 PM
-- Design Name:
-- Module Name: register - Behavioral
-- Project Name:
-- Target Devices:
-- Tool Versions:
-- Description:
--
-- Dependencies:
--
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
--
----------------------------------------------------------------------------------


library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity registerB is
    port(
        C: in unsigned(15 downto 0);
        LDB: in STD_logic;
        reset: in STD_logic;
        clk: in STD_LOGIC;
        B: out unsigned(15 downto 0)
    );
end registerB;

architecture behaviour of registerB is
begin
    process(clk, reset)
    begin
        if reset = '1' then
            B <= "0000000000000000";
        elsif rising_edge (clk) then
            if LDB = '1' then
                B <= C;
            end if;
        end if;
    end process;
end behaviour;
